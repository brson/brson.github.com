build:
    ./ssg/target/debug/ssg build

clean:
    ./ssg/target/debug/ssg clean

serve: build
    python3 -m http.server 4001 -d _site
