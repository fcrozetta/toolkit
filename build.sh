declare -a progs=("fc-json")
mkdir -p bin
for prog in "${progs[@]}"
do
    cd $prog
    cargo build --release
    cp ./target/release/$prog bin/
    cd ..
done
cd bin
zip ../toolkit.zip *