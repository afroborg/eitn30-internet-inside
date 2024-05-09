base_address = 0
base_channel = 108

mobile_address = 1
mobile_channel = 116

delay = 5

run-mobile:
	sudo ./eitn30-internet-inside \
		--receiver-address 		$(base_address) \
		--transmitter-address 	$(mobile_address) \
		--receiver-channel 		$(mobile_channel) \
		--transmitter-channel 	$(base_channel) \
		--delay 				$(delay) \
		--transmitter-gpio		7 \
		--receiver-gpio			17
run-base:
	sudo ./eitn30-internet-inside \
		--receiver-address 		$(mobile_address) \
		--transmitter-address 	$(base_address) \
		--receiver-channel 		$(base_channel) \
		--transmitter-channel 	$(mobile_channel) \
		--forward 				eth0 \
		--forward 				wlan0 \
		--delay					$(delay) \
		--transmitter-gpio		17 \
		--receiver-gpio			27