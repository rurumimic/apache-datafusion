# Examples

- github: [apache/arrow-testing](https://github.com/apache/arrow-testing)

## Init

```bash
cargo add --dev datafusion
cargo add --dev tokio --features rt-multi-thread
```

## Test

### Download Test Data

```bash
mkdir data
```

```bash
curl -L -o data/aggregate_test_100.csv \
https://raw.githubusercontent.com/apache/arrow-testing/master/data/csv/aggregate_test_100.csv
```

