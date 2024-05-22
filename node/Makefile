PROFILE = yokohama

FUNK_NAME = node

CITY = $(shell cat .env | grep CITY | awk -F'=' '{print $$2}')
APIKEY = $(shell cat .env | grep APIKEY | awk -F'=' '{print $$2}')

build:
	npm run build

fn-create:
	aws lambda create-function --function-name $(FUNK_NAME) --runtime "nodejs18.x" --role arn:aws:iam::465254350166:role/lambda-ex --zip-file "fileb://dist/index.zip" --handler index.handler --profile $(PROFILE)

fn-list:
	aws lambda list-functions --query "Functions[*].[FunctionName, Runtime]" --output table --profile $(PROFILE)

fn-deploy:
	aws lambda update-function-code --function-name $(FUNK_NAME) --zip-file "fileb://dist/index.zip" --profile $(PROFILE)

fn-invoke:
	aws lambda invoke --function-name $(FUNK_NAME) --payload '{}' --profile $(PROFILE) output.txt > /dev/null 2>&1 && cat output.txt && rm output.txt 

curl:
	curl "https://api.openweathermap.org/data/2.5/weather?q=$(CITY)&units=metric&appid=$(APIKEY)"

sam:
	sam local invoke NodeFunction --env-vars env.json
