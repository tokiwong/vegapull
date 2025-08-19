package main

import (
	"log"
	"os"
	"os/exec"
	"strings"
	"sync"
)

const (
	imagesDir = "data/english/images"
)

func main() {
	_, err := os.Stat("data/english")
	if err != nil {
		log.Fatalf("Data directory %s does not exist: %v\n", imagesDir, err)
	}

	log.Printf("Unpacking data from %s...\n", imagesDir)
	// get list of zipped files in the data directory
	files, err := os.ReadDir(imagesDir)
	if err != nil {
		log.Fatalf("Failed to read directory %s: %v\n", imagesDir, err)
	}

	var wg sync.WaitGroup
	errChan := make(chan error, len(files))

	for _, file := range files {
		if file.IsDir() || !strings.HasSuffix(file.Name(), ".zip") {
			continue // skip directories and non-zip files
		}

		wg.Add(1)
		go func() {
			defer wg.Done()
			filePath := imagesDir + "/" + file.Name()
			outputDir := imagesDir + "/" + strings.TrimSuffix(file.Name(), ".zip")
			log.Printf("Unpacking %s to %s...\n", filePath, outputDir)

			cmd := exec.Command("unzip", "-o", filePath, "-d", outputDir)
			if err := cmd.Run(); err != nil {
				errChan <- err
				return
			}

			log.Printf("Unpacked %s successfully.\n", file.Name())

			if err = os.Remove(filePath); err != nil {
				errChan <- err
				return
			}
		}()
	}

	wg.Wait()
	close(errChan)
	for err := range errChan {
		if err != nil {
			log.Fatalf("Error during unpacking: %v\n", err)
		}
	}
}
