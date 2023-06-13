import asyncio
import sys

from viam.robot.client import RobotClient
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
    location = sys.argv[1]
    secret = sys.argv[2]

    robot = await connect(location, secret)

    print('Resources:')
    print(robot.resource_names)

    await robot.close()

if __name__ == '__main__':
    print("err muh grrr")
    asyncio.run(main())
 
