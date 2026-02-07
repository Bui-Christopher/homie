### Zillow Datasets

## Definitions Zillow/ZHVI
**Zillow Home Value Index (ZHVI):** A measure of the typical home value and market changes across a given region and housing type. It reflects the typical value for homes in the 35th to 65th percentile range. Available as a smoothed, seasonally adjusted measure and as a raw measure.

Zillow publishes top-tier ZHVI ($, typical value for homes within the 65th to 95th percentile range for a given region) and bottom-tier ZHVI ($, typical value for homes within the 5th to 35th percentile range for a given region).

Zillow also publishes ZHVI for all single-family residences ($, typical value for all single-family homes in a given region), for condo/coops ($), for all homes with 1, 2, 3, 4 and 5+ bedrooms ($).

**Zillow Home Value Forecast (ZHVF):** A month-ahead, quarter-ahead and year-ahead forecast of the Zillow Home Value Index (ZHVI). ZHVF is created using the all homes, mid-tier cut of ZHVI and is available both raw and smoothed, seasonally adjusted.

**Single-family Residences (SFR):** Representing the typical price of detached homes, a key housing market indicator Zillow tracks alongside other segments like condos and multi-family units, using data from Zestimates to show trends in housing value for standalone properties.

[ZHVI Methodology](https://www.zillow.com/research/methodology-neural-zhvi-32128/)

## Dataset fields
* ZHVI All Homes - RegionID, SizeRank, RegionName, RegionType, StateName, State, City, Metro, CountyName, <Date>
* Mortgage Payment - RegionID, SizeRank, RegionName, RegionType, StateName, <Date>
* Total Monthly Payment - RegionID, SizeRank, RegionName, RegionType, StateName, State, <Date> 
* ZHVF Forecast All Homes - RegionID, SizeRank, RegionName, RegionType, StateName, State, City, Metro, CountyName, BaseDate, <Date>

## Access method (API, CSV File)
Zillow only provides CSV Files for research and academic purposes using this [link.](https://www.zillow.com/research/data/)
There is an API available through **bridgedataoutput.com** that Zillow provides their Zestimates to. We have submitted a request to Zillow and are awaiting response. 

## Pros/Cons
### API
#### Pros
* Live Data
* API call and forget
* Zillow API Reference for ease of use

#### Cons
* Have to be a business
* Have to follow ToS

### CSV File
#### Pros
* Free to use

#### Cons
* Have to upkeep every 2 weeks
* Have to automate process
* Have to follow ToS

## Conclusion
After consideration and communication with Zillow, they have only allowed our team to utilize their CSV files. We must go through the **CSV File** method.
