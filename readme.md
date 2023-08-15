<div align="center">
  <h1><code>rover-cli</code></h1>
  <strong>
    A coding exercise in controlling a set of rovers on mars.
  </strong>
</div>
<br><br>

## 🛠 Installation

```sh
cargo install rover-cli
```

## 🔋 Usage

**Print output to console:**

```sh
rover-cli foo.csv
```

**Save output to file:**
```sh
rover-cli --output output.csv foo.csv
```

**To see helpful information:**

```sh
rover-cli --help
```

## 💭 Code Choices

 - `--unbounded` command flag is included to allow to the rover to exit the plateau.
 - `isize` is used to represent co-ordinates to allow the rover to pass (0, 0).
 - `fs::read_to_string` is used for simplicity over the more performant but more complex `BufReader`.
 - `RoverControlSatellite` is used for themeatic effect!

## 🔬 Testing

To run tests for the CLI:

```sh
cargo test
```

### Testing Approach

Unit tests that cover basic functionality and possible branches are included, however as this is a coding exercise, 100% coverage has not been aimed for. 

### Generate Coverage Report

**Setup**

```sh
rustup component add llvm-tools-preview &&
cargo install cargo-llvm-cov
```

**Usage**

To create a coverage report:

```sh
cargo llvm-cov
```

To debug a coverage report:

```sh
cargo llvm-cov --html --output-dir coverage
```