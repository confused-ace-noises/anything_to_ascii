ask_yes_no() {
    local prompt="$1"   # The prompt message
    local default="$2"
    local response

    # Loop until valid input is given
    while true; do
        read -p "$prompt" response
        response=${response:-$default}  # Use default if no input

        # Convert response to lowercase
        response=$(echo "$response" | tr '[:upper:]' '[:lower:]')

        
        if [[ "$response." =~ ^(yes|y)$ ]]; then
            return 0   # Return 0 for 'yes'
        elif [[ "$response" =~ ^(no|n)$ ]]; then
            return 1   # Return 1 for 'no'
        else
            echo "Invalid input, please enter y or n."
        fi
    done
}

if ask_yes_no "This will install the `git`, `ffmpeg` and `rust` packages if not present. Continue? [Y/n]" "y"; then 
    echo "WARNING! This may take a while."
else 
    echo "refused to install required packages. Exiting..."
    exit 0
done


echo "executing git existence check."

if command -v git > /dev/null 2>&1; then
    echo "git is installed. continuing..."
else
    echo "git is not installed."
    echo "installing git..."
    pkg install -y git
fi

if ! command -v git > /dev/null 2>&1; then
    echo "failed to install git! required manual intervention."
    exit 1
fi 


echo "executing ffmpeg existence check."

if command -v ffmpeg > /dev/null 2>&1; then
    echo "ffmpeg is installed. continuing..."
else
    echo "ffmpeg is not installed."
    echo "installing ffmpeg..."
    pkg install -y ffmpeg
fi

if ! command -v ffmpeg > /dev/null 2>&1; then
    echo "failed to install ffmpeg! required manual intervention."
    exit 1
fi 


echo "executing cargo existence check."

if command -v cargo > /dev/null 2>&1; then
    echo "cargo is installed. continuing..."
else
    echo "cargo is not installed."
    echo "installing rust..."
    pkg install -y rust
fi

if ! command -v cargo > /dev/null 2>&1; then
    echo "failed to install rust! required manual intervention."
    exit 1
fi 

echo "all download and install dependencies OK. continuing to download..."

git clone https://github.com/confused-ace-noises/anything_to_ascii.git

if ! ls | grep "anything_to_ascii"; then 
    echo "failed to clone anything_to_ascii repository! manual intervention is required."
    exit 1
fi

echo "successfully cloned repository. building anything_to_ascii..."

cargo build --release --manifest-path anything_to_ascii

if [ $? -ne 0 ]; then
    echo "building failed! manual intervention is required."
    exit 1
fi

cp anything_to_ascii/target/release/anything_to_ascii .

chmod +x anything_to_ascii

if ask_yes_no "Do you want to delete the build files? [Y/n]" "y"; then
    rm -rf anything_to_ascii/

    if [ $? -ne 0 ]; then
        echo "deleting failed! manual intervention is required."
        exit 1
    fi
fi



if ask_yes_no "Do you want to move the executable to $PREFIX/bin (where all the other applications are)? [Y/n]" "y"; then
    mv anything_to_ascii $PREFIX/bin

    if [ $? -ne 0 ]; then
        echo "moving failed! manual intervention is required."
        exit 1
    fi
fi

exit 0