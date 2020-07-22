local:
	source ./scripts/.env_local ; \
	docker-compose -f ./deployment/docker-compose.yaml up -d postgres adminer ; \
	sleep 5 ; \
	cd domain ; \
	diesel migration run ; \
	cd .. : \
	cd server && cargo run

local-down:
	source ./scripts/.env_local ; \
	docker-compose -f ./deployment/docker-compose.yaml down; 