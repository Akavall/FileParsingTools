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
