#!/bin/bash
# Make should be run before running tests

host="lundrigan.byu.edu"
port=1883
netid=xemkis;
action=reverse;
message="This is not the greatest song in the world. This is just a tribute";

# Test command args/opts
echo -en "\n~~~~~~~~~Insufficient args test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v "$netid";
echo -en "\n~~~~~~~~~Excessive args test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v "$netid" "$action" "$message" "extra_arg";
echo -en "\n~~~~~~~~~Unrecognized opt test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v -l "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Invalid port num test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v -p "40lettuce" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Default port/host test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Opt port/host test~~~~~~~~~\n"
sudo ./target/debug/mqtt_client -v -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Reverse test~~~~~~~~~\n"
action="reverse"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Uppercase test~~~~~~~~~\n"
action="uppercase"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Lowercase test~~~~~~~~~\n"
action="lowercase"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Shuffle test~~~~~~~~~\n"
action="shuffle"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Reverse test~~~~~~~~~\n"
action="reverse"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Title-case test~~~~~~~~~\n"
action="title-case"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Long-message test~~~~~~~~~\n"
message="Ever since I was a young boy I played the silver ball. From soho down to brighton, I must've played them all. But I ain't seen nothing like him in any amusement hall. that deaf dumb and blind kid sure plays a mean pinball"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Long-message test~~~~~~~~~\n"
message="Twas brillig and the slithy toves did gyre and gimbal in the wabe. All mimsy was the borogrove and the momewraths outgrabe. \"Beware the jabberwock my son, the jaws that byte, the claws that catch. Beware the jubjub bird and shun the frumious bandersnatch.\" He took his vorpal sword in hand long-time the manxom foe he sought; so rested he by the tum-tum tree and stood a while in thought. And as in uffish thought he stood, the jabberwock with eyes of flame came whiffling through the tolgy wood, and burbled as it came. One, two, one, two! and through and through! his vorpal blade went snicker-snack. He left it dead, and with it's head he went galumphing back. \"And hast though slain the jabberwock? Come to my arms my beamish boy! Oh frabjious day! Calooh, callay!\" he chortled in his joy. Twas brillig and the slithy toves did gyre and gimbal in the wabe. All mimsy was the borogrove and the momewraths outgrabe"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Different net-id test~~~~~~~~~\n"
netid=le0nh4rt
sudo ./target/debug/mqtt_client -v -p "$port" -h "$host" "$netid" "$action" "$message";
echo -en "\n~~~~~~~~~Bad Action test~~~~~~~~~\n"
action="nottacase"
sudo ./target/debug/mqtt_client -p "$port" -h "$host" "$netid" "$action" "$message";
