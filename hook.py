import asyncio
import sys
import time

from viam.robot.client import RobotClient
from viam.components.board import Board
from viam.rpc.dial import Credentials, DialOptions

async def connect(location, secret):
    creds = Credentials(
        type='robot-location-secret',
        payload=secret)
    opts = RobotClient.Options(
        refresh_interval=0,
        dial_options=DialOptions(credentials=creds)
    )
    return await RobotClient.at_address(location, opts)

async def main():
    print("err muh grrr")
    location = sys.argv[1]
    secret = sys.argv[2]
    target = sys.argv[3]

    robot = await connect(location, secret)

    print('Resources:')
    resources = robot.resource_names
    print(resources)

    # get pin, set high with pwm of 0.5

    await robot.close()

if __name__ == '__main__':
    asyncio.run(main())
 
