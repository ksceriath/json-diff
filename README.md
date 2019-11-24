# json-diff

json-diff is a command line utility to compare two jsons.  

Input can be fed as inline strings or through files.  
For readability, output is neatly differentiated into three categories: keys with different values, and keys not present in either of the objects.  
Only missing or unequal keys are printed in output to reduce the verbosity.

## Screenshot of diff results

[![A screenshot of a sample diff with json_diff](https://github.com/ksceriath/json-diff/blob/master/Screenshot.png)](https://github.com/ksceriath/json-diff/blob/master/Screenshot.png)

Usage Example:

`$ json_diff f source1.json source2.json`  
`$ json_diff d '{...}' '{...}'`

Option:

f   :   read input from json files  
d   :   read input from command line


