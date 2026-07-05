package zillow

import (
	"time"
	"encoding/csv"
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
)
func (d Date) String() string {
	return d.Format("2006-01-02")
}

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

	// metadata columns end here (Zillow standard is 8)
	metaCols := 10

	dateHeaders := header[metaCols:]

	dates := make([]time.Time, len(dateHeaders))
	for i, h := range dateHeaders {
		t, err := time.Parse("2006-01-02", h)
		if err != nil {
			return nil, fmt.Errorf("bad date header %q: %w", h, err)
		}
		dates[i] = t
	}
	// Map column name -> index, so we don't rely on position
	colIndex := make(map[string]int)
	for i, name := range header {
		colIndex[name] = i
	}

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

		listing := Listing {
				SizeRank:   sizeRank,
				RegionName: row[colIndex["RegionName"]],
				RegionType: row[colIndex["RegionType"]],
				StateName:  row[colIndex["StateName"]],
				State:      row[colIndex["State"]],
				City:       row[colIndex["City"]],
				Metro:      row[colIndex["Metro"]],
				CountyName: row[colIndex["CountyName"]],
		}

		for i, d := range dates {
			if metaCols+i >= len(row) {
				continue
			}

			val := row[metaCols+i]
			if val == "" {
				continue
			}

			growth, err := strconv.ParseFloat(val, 64)
			if err != nil {
				continue
			}
			
			listing.History = append(listing.History, GrowthPoint{
				Date: Date{Time: d},
				Growth: growth,
			})
		}

		listings = append(listings, listing)
	}

	return listings, nil
}
