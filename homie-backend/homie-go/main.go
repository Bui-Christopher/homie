package main

import ( 
	"fmt"
	"log"
	"homie/zillow"
)

func main() {
	listings, err := zillow.ReadCSV("testdata/Zip_zhvf_growth_uc_sfrcondo_tier_0.33_0.67_sm_sa_month.csv")
	if err != nil {
		log.Fatalf("failed to read CSV: %v", err)
	}

	fmt.Printf("Loaded %d listings\n", len(listings))
	
	// Print the first few to eyeball the results
	for i, l := range listings{
		if i >= 5 {
			break
		}
		fmt.Printf("%+v\n", l)
	}
}
