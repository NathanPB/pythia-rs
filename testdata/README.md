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
