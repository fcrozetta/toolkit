declare -a progs=("fc-json")
mkdir -p artifacts
for prog in "${progs[@]}"
do
    cd $prog
    cargo build --release
    cd ./target/release/$prog
    zip $prog
    cd -
    cp ./target/release/$prog.zip ../artifacts/
    cd ..
done