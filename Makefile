base_number = 32
mobile_number = 24

base_ip = 100.93.60.11
base_lab_ip = inuti32.lab.eit.lth.se
mobile_ip = 100.111.73.55
mobile_lab_ip = inuti24.lab.eit.lth.se

build_target = aarch64-unknown-linux-gnu

connect-base:
	ssh -i ~/.ssh/eitn30-pi pi@$(base_ip)
connect-base-lab:
	ssh -i ~/.ssh/eitn30-pi pi@$(base_lab_ip)
connect-mobile:
	ssh -i ~/.ssh/eitn30-pi pi@$(mobile_ip)
connect-mobile-lab:
	ssh -i ~/.ssh/eitn30-pi pi@$(mobile_lab_ip)
build:
	sh scripts/build.sh $(build_target)
deploy-base:
	sh scripts/deploy.sh $(base_number) $(base_ip) --base
deploy-mobile:
	sh scripts/deploy.sh $(mobile_number) $(mobile_ip) --mobile
deploy: deploy-base deploy-mobile
test: 
	sh scripts/test.sh $(build_target)
pdf:
	pandoc individual-goals.md -o individual-goals.pdf
	pandoc individual-goals-updated.md -o individual-goals-updated.pdf
	pandoc performance-evaluation.md -o performance-evaluation.pdf