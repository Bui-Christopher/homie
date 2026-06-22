package zillow

import (
	"encoding/csv"
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
)

func ReadCSV(filePath string) ([]Listing, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, fmt.Errorf("could not open file: %w", err)
	}
	defer file.Close()

	reader := csv.NewReader(file)

	header, err := reader.Read()
	if err != nil {
		return nil, fmt.Errorf("could not read header: %w", err)
	}

	// Map column name -> index, so we don't rely on position
	colIndex := make(map[string]int)
	for i, name := range header {
		colIndex[name] = i
	}

	// Assume the last column is the most recent date's value
	valueIndex := len(header) - 1

	var listings []Listing

	for {
		row, err := reader.Read()
		if errors.Is(err, io.EOF) {
			break
		}
		if err != nil {
			return nil, fmt.Errorf("error reading row: %w", err)
		}

		sizeRank, err := strconv.Atoi(row[colIndex["SizeRank"]])
		if err != nil {
			continue // skip rows with a bad SizeRank
		}

		value, err := strconv.ParseFloat(row[valueIndex], 64)
		if err != nil {
			continue // skip rows with missing/bad value data
		}

		listing := Listing {
				SizeRank:   sizeRank,
				RegionName: row[colIndex["RegionName"]],
				RegionType: row[colIndex["RegionType"]],
				StateName:  row[colIndex["StateName"]],
				State:      row[colIndex["State"]],
				City:       row[colIndex["City"]],
				Metro:      row[colIndex["Metro"]],
				CountyName: row[colIndex["CountyName"]],
				Value:      value,
		}

		listings = append(listings, listing)
	}

	return listings, nil
}
