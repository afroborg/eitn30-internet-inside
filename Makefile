base_number = 06
mobile_number = 24

connect-base:
	ssh -i ~/.ssh/eitn30-pi pi@inuti$(base_number).lab.eit.lth.se
connect-mobile:
	ssh -i ~/.ssh/eitn30-pi pi@inuti$(mobile_number).lab.eit.lth.se
build:
	sh scripts/build.sh
deploy-base:
	sh scripts/deploy.sh -n $(base_number)
deploy-mobile:
	sh scripts/deploy.sh -n $(mobile_number)
deploy: deploy-base deploy-mobile