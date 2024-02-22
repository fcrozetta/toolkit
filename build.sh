declare -a progs=("fc-json")
mkdir -p bin
for prog in "${progs[@]}"
do
    cd $prog
    cargo build --release --target aarch64-apple-darwin
    cp ./target/aarch64-apple-darwin/release/$prog ../bin/
    cd ..
done
cd bin
zip ../toolkit.zip *