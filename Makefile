base_number = 06
mobile_number = 24

base_ip = 100.124.31.24
mobile_ip = 192.168.0.109

connect-base:
	ssh -i ~/.ssh/eitn30-pi pi@$(base_ip)
connect-mobile:
	ssh -i ~/.ssh/eitn30-pi pi@$(mobile_ip)
build:
	sh scripts/build.sh
deploy-base:
	sh scripts/deploy.sh -n $(base_number)
deploy-mobile:
	sh scripts/deploy.sh -n $(mobile_number)
deploy: deploy-base deploy-mobile