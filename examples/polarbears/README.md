# Polar Bears

Renders the USGS Polar Bears [JSON dataset](https://polar-bears.vercel.app/polar-bears/USGS_WC_eartags_output_files_2009-2011-Status) as an HTML table.

```sh
$ curl -s https://polar-bears.vercel.app/polar-bears/USGS_WC_eartags_output_files_2009-2011-Status.json | \
  temple \
  --format=json \
  --templates=templates \
  --output=polarbears.html --force \
  templates/main.html
```
