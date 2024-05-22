import { Context, APIGatewayProxyResult, APIGatewayEvent } from 'aws-lambda';
import fetch from 'node-fetch';

import * as dotenv from 'dotenv';
dotenv.config();

// OpenWeather
// https://home.openweathermap.org/api_keys
const	URL = `https://api.openweathermap.org/data/2.5/weather?q=${process.env.CITY}&units=metric&appid=${process.env.APIKEY}`

interface WeatherResponse {
  name: string;
  main: {
    temp_max: number;
    temp_min: number;
  },
  wind: {
    speed: number;
  }
}

export const handler = async (event: APIGatewayEvent, context: Context): Promise<APIGatewayProxyResult> => {
  console.log(`Event: ${JSON.stringify(event, null, 2)}`);
  console.log(`Context: ${JSON.stringify(context, null, 2)}`);

  try {
    const response = await fetch(URL);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    const data = await response.json() as WeatherResponse;

    return {
      statusCode: 200,
      body: JSON.stringify({
        message: {
          "都市": data.name,
          "最高気温": `${data.main.temp_max}℃`,
          "最低気温": `${data.main.temp_min}℃`,
          "風速": `${data.wind.speed}M`,
        }
      }),
    };
  } catch (error) {
    return {
      statusCode: 500,
      body: JSON.stringify({
        message: 'Error fetching data',
        error: error.message,
      }),
    }
  }
};
