# HeadColor
Rust learning project, creates a binary file that works similar to head on linux machines, but we add color to specified column.

Instalation:

To link statically: 

```
rustc head_color.rs
sudo mv head_color /usr/bin
```

To link dynamically, creates a smaller executable:

```
rustc -C prefer-dynamic head_color.rs
sudo mv head_color /usr/bin
```

Usage:
```
head_color -file_name -column_number -delim -number_of_rows_to_show
```

Example:
```
head_color my_file.txt 2 "," 5
```

Will show first 5 rows including the header; all elements in the second column will be colored green.
The delimiter is needed for the function to know where the columns are.

**Using col_counter:**

```
rustc head_color.rs
sudo mv col_counter /usr/bin
```

Usage:
```
col_counter -file_name -column_number -delim -top_n_most_common_items
```

Example:
```
col_counter my_file.txt 2 "," 5
```

Will display ranks, and counts corresponding to 5 most frequently occuring items.

**Using set_func:**

Only applies to two files that have single columns.

```
rustc set_func.rs
sudo mv set_func /usr/bin
```

Usage:
```
set_func -file_1_name -file_1_name -function_type
```

function_tupe:
```
-u <- union
-i <- intersection
-d <- difference
```

Example:
To print the results:
```
set_func file_1.txt file_2.txt -u
```

To pipe the results to another file
```
set_func file_1.txt file_2.txt -u > file_3.txt
```