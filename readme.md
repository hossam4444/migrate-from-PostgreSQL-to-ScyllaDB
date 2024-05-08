# Migrate to ScyllaDB 

## Steps i am following 

### 1: Working to build an architecture that guarantees a high-performance read and write even with big data. In Progress

### 2: writing some functions to operate over the ScyllaDB using rust driver. in progress

### 3: Test performence in the production. not started

## how to run
- clone
- navigate to the project directory
- create .env file and set SCYLLA_URI var with Ip of the DB host and port like so ```SCYLLA_URI="127.0.0.1:9042"```
- in your terminal run ```cargo run```
- now execute the desired function to effect your DB