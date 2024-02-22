declare -a progs=("fc-json")
mkdir -p bin
for prog in "${progs[@]}"
do
    cd $prog
    cargo build --release
    cp $prog/target/release/$prog bin/
done