# Files

## `DSSAT-Soils.shp.zip`
Subset of the dataset [Global High-Resolution Soil Profile Database for Crop Modeling Applications](https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/1PEEY0), cut into lon/lat 12,12 to 15,15 to reduce file size.

Steps to create:
1. Download the original file from the Harvard Dataverse. Access the link above and seek for the file ``Point5m_SoilGrids-for-DSSAT-10km_v1.shp.zip``.
2. Unzip it:

```sh
unzip Point5m_SoilGrids-for-DSSAT-10km_v1.shp.zip
```

3. Use OGR to cut the file:

```sh
ogr2ogr -f "ESRI Shapefile" DSSAT-Soils point5m_soilgrids-for-dssat-10km_v1.shp -spat 12 12 15 15
```

4. Zip it back up:

```sh
zip -j DSSAT-Soils.shp.zip DSSAT-Soils/*
```

The final file is `DSSAT-Soils.shp.zip`.

## `DSSAT-Soils.tif`

Rasterized version of [DSSAT-Soils.shp.zip](#dssat-soilsshpzip)

### Steps to create

2. Rasterize it using GDAL:

```sh
gdal_rasterize -a CELL5M -tr 0.0833 0.0833 -ot Int32 -of GTiff /vsizip/DSSAT-Soils.shp.zip DSSAT-Soils.tif
```

The final file is `DSSAT-Soils.tif`.

> **Note:** In the case of this example file, the resolution is known to be 5 arc-minute (stated in the dataset description), so the -tr flag values are set to 0.0833. For other resolutions, the values may need to be adjusted.
>
> Ideal pixel size formula:
> ```
> Pixel Size (X) = (max_x - min_x) / sqrt(N)
> Pixel Size (Y) = (max_y - min_y) / sqrt(N)
> where max_x, min_x, max_y, min_y = the bounding box coordinates; (ogrinfo -al -so DSSAT-Soils.shp.zip | grep "Extent");
>       N = total number of points (ogrinfo -al -so DSSAT-Soils.shp.zip | grep "Feature Count").
> ```
> 
> Irregular shapes might lead to a less precise pixel size, so manual intervention might be necessary.
