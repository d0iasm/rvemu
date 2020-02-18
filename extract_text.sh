IN_F=./sample_binary/hello
OUT_F=./sample_binary/hello_text
SECTION=.text

objdump -h $IN_F |
    grep $SECTION |
    awk '{print "dd if='$IN_F' of='$OUT_F' bs=1 count=$[0x" $3 "] skip=$[0x" $6 "]"}' |
    bash
