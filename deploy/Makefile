base_address = 0
base_channel = 108

mobile_address = 1
mobile_channel = 116

longge_default = false
auto_ack = true

run-mobile:
	sudo ./eitn30-internet-inside \
		--receiver-address 		$(base_address) \
		--transmitter-address 	$(mobile_address) \
		--receiver-channel 		$(mobile_channel) \
		--transmitter-channel 	$(base_channel) \
		--transmitter-gpio		7 \
		--receiver-gpio			17 \
		--longge-default		$(longge_default) \
		--auto-ack				$(auto_ack)
run-base:
	sudo ./eitn30-internet-inside \
		--receiver-address 		$(mobile_address) \
		--transmitter-address 	$(base_address) \
		--receiver-channel 		$(base_channel) \
		--transmitter-channel 	$(mobile_channel) \
		--forward 				eth0 \
		--forward 				wlan0 \
		--transmitter-gpio		17 \
		--receiver-gpio			27 \
		--auto-ack				$(auto_ack)
