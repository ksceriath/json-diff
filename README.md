# json-diff

json-diff is a command line utility to compare two jsons.  

Input can be fed as inline strings or through files.  
For readability, output is neatly differentiated into three categories: keys with different values, and keys not present in either of the objects.  
Only missing or unequal keys are printed in output to reduce the verbosity.

Usage Example:

`$ json_diff file source1.json source2.json`  
`$ json_diff direct '{...}' '{...}'`

Option:

file   :   read input from json files  
direct   :   read input from command line

### Installation

Currently, json-diff is available through crates.io (apart from building this repo directly). For crate installation,  
* Install cargo, through rustup  
`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  
* Install json-diff  
`$ cargo install json_diff`


