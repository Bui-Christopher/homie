# Dataset Reader

Writing the dataset reader in Golang. Development will occur whilst Jose is learning Golang, this README and **homie-go** is subject to change.

## Notes
**02/07/2026**
Currently learning about how to build an application in Golang. So far, I have created a Go-idiomatic directory tree to help avoid abstraction. Each directory folder is to separate application code. I've initialized the folder into a go.mod and created my main.go file that will call upon the rest of the application code. I've only implemented a single println to make sure it works. I'm currently working on building a struct and trying to understand what fields will be required and optional.

**06/21/2026**
Have restarted my progress and wiped my previous directory structure. I will keep the same principles as before but I am tightening my workload and will expand if needed.

**06/22/2026**
Created the first version of the dataset reader! I have reworked my directory but am still keeping the same Go-idiomatic tree style. In this latest version, I've created a "Zillow" folder and package to separate MLS sources as we move forward. The struct is included within the package as well as the reader itself. As of now the reader is reading all data but I wasn't able to display the date correctly. Am listing only 5 items at a time.

**07/05/2026**
Updated the model and reader to accept separate dates from the headers and also display the growth of the area on that date. Had to create a new func specifically to declare the date and then use a map and index to display the dates with their growth in the same listing. Move the length of the map if needed according to how many columns you are using.
