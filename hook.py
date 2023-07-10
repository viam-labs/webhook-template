import asyncio
import sys
import time
import logging
from datetime import timedelta

from viam.robot.client import RobotClient
from viam.components.board import Board
from viam.rpc.dial import Credentials, DialOptions
from viam.proto.component.board import PowerMode

# Number of times each rpc-based call will be tried
NUM_TRY = 3

async def connect(location, secret):
    creds = Credentials(
        type='robot-location-secret',
        payload=secret)
    opts = RobotClient.Options(
        refresh_interval=0,
        dial_options=DialOptions(credentials=creds),
        attempt_reconnect_interval=15
    )
    return await RobotClient.at_address(location, opts)

async def try_to_async(func, args, error, num_try=NUM_TRY):
    r = None
    for _ in range(num_try):
        try:
            r = await func(*args)
            break
        except:
            logging.error(error)
    return r

def try_to(func, args, error, num_try=NUM_TRY):
    r = None
    for _ in range(num_try):
        try:
            r = func(*args)
            break
        except:
            logging.error(error)
    return r


async def main():
    location = sys.argv[1]
    secret = sys.argv[2]
    print(f"location: {location}")
    print(f"secret: {secret}")
    time.sleep(3)

    # duration of deep sleep until device's next call to webhook
    delta = timedelta(
        #days=50,
        #seconds=10,
        #microseconds=10,
        #milliseconds=29000,
        minutes=1,
        #hours=8,
        #weeks=2
    )

    # in the event of unstable connection, every gRPC-based call has the
    # possibility of timing out and throwing an exception.
    robot = await try_to_async(connect, [location, secret], "failed to connect")
    board = try_to(Board.from_robot, [robot, "board"], "connection dropped while getting board")
    gpio = await try_to_async(board.gpio_pin_by_name, ["15"], "connection dropped while getting gpio")
    await try_to_async(gpio.set, [True], "connection dropped while setting pin high")

    time.sleep(3)

    await try_to_async(gpio.set, [False], "connection dropped while setting pin low")

    # The final method makes a call that expects a response in order to return properly,
    # however, if the power is successfully set to Deep Sleep, the device will not be able
    # to respond, throwing an exception.
    await try_to_async(board.set_power_mode, [PowerMode.POWER_MODE_OFFLINE_DEEP, delta], "success", 1)

    # As the connection would time out if successfully put to deep sleep, `robot.close()`
    # is not necessary and would fail.

if __name__ == '__main__':
    asyncio.run(main())
    sys.exit()
 
