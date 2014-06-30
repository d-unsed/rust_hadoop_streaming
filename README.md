# Hadoop Strteaming using Rust

### Description

An example of using Hadoop Streaming in Rust.

This is a MapReduce program for finding maximum global temperatures by year
from [NCDC](http://ncdc.noaa.gov/) weather records.
It's another example of using Hadoop Streaming for
[Hadoop: The Definitive Guide](http://www.amazon.com/Hadoop-Definitive-Guide-Tom-White/dp/1449311520)
book.

Developed against Rust v0.10

### Usage

1. Compile mapper and reducer
```bash
$ rustc mapper/ncdc_mapper.rs && rustc reducer/ncdc_reducer.rs
```

2. (optional) Check whether everything works fine
```bash
$ cat samples/sample.txt | ./ncdc_mapper | ./ncdc_reducer
```

3. Run hadoop
```bash
$ hadoop jar $HADOOP_INSTALL/hadoop-streaming-*.jar \
             -input ncdc_data \
             -output output \
             -mapper ncdc_mapper \
             -reducer ncdc_reducer
```

4. ...

5. PROFIT! :)
