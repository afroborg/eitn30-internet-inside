base_number = 32
mobile_number = 24

base_ip = 100.93.60.11
mobile_ip = 100.65.157.26

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