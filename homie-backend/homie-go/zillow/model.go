package zillow

import (
	"time"
)

type Date struct{
	time.Time
}

type GrowthPoint struct{
	Date Date
	Growth float64
}

type Listing struct {
	SizeRank int
	RegionName string
	RegionType string
	StateName string
	State string
	City string
	Metro string
	CountyName string

	History []GrowthPoint
}
