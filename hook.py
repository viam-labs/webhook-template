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

    board = Board.from_robot(robot, target)
    board_pin = await board.gpio_pin_by_name("22")
    print(f"gpio_pin_by_name return value: {await board_pin.get()}")
    loop {
        await board_pin.set(True)
        time.sleep(1)
        await board_pin.set(False)
        time.sleep(1)

    }

    # get pin, set high with pwm of 0.5

    await robot.close()

if __name__ == '__main__':
    asyncio.run(main())
 
