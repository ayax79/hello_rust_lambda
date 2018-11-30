#!/usr/bin/env zsh


if [[  $(uname) = 'Darwin' ]]
then 
    # if the there is no sym link to the linux-musl linker create it
    if [[  ! -f $(which musl-gcc)  &&  -f /usr/local/bin/x86_64-linux-musl-gcc  ]]
    then 
        echo "linking musl-gcc"
        ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
    fi

    # if the cargo config doesn't contain the correct linker then add it
    $(grep -q linux-musl-gcc ./.cargo/config > /dev/null 2>&1)
    if [[  "$?" -ne "0" ]]
    then
        echo "Setting the linker to linux-musl-gcc"
        mkdir ./.cargo
        cat << eos > ./.cargo/config
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
eos
    fi
fi

cargo build --release --target x86_64-unknown-linux-musl && \
    zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap