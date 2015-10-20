# Hadoop Streaming using Rust

### Description

An example of using Hadoop Streaming in Rust.

This is a MapReduce program for finding maximum global temperatures by year
from [NCDC](http://ncdc.noaa.gov/) weather records.
It's another example of using Hadoop Streaming for
[Hadoop: The Definitive Guide](http://www.amazon.com/Hadoop-Definitive-Guide-Tom-White/dp/1449311520)
book.

Developed against Rust v1.3. You can find previous versions
(idk why you could want :smile:) for pre-release Rust at the following branches:

 - [rust-0.10](https://github.com/d-unseductable/rust_hadoop_streaming/tree/rust-0.10)
 - [rust-0.11](https://github.com/d-unseductable/rust_hadoop_streaming/tree/rust-0.11)
 - [rust-0.12](https://github.com/d-unseductable/rust_hadoop_streaming/tree/rust-0.12)

### Usage

1. Compile mapper and reducer

  ```bash
  $ rustc -O src/mapper.rs && rustc -O src/reducer.rs
  ```

2. (optional) Check whether everything works fine

  ```bash
  $ cat samples/sample.txt | ./mapper | ./reducer
  ```

3. Run hadoop

  ```bash
  $ hadoop jar $HADOOP_INSTALL/hadoop-streaming-*.jar \
               -input   ncdc_data \
               -output  output \
               -mapper  mapper \
               -reducer reducer
  ```

4. ...

5. PROFIT! :)
