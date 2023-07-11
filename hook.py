import asyncio
import sys
import time
import logging
from datetime import timedelta

from viam.robot.client import RobotClient
from viam.components.board import Board
from viam.rpc.dial import Credentials, DialOptions
from viam.proto.component.board import PowerMode

async def connect(location, secret):
    creds = Credentials(
        type='robot-location-secret',
        payload=secret)
    opts = RobotClient.Options(
        refresh_interval=0,
        dial_options=DialOptions(credentials=creds),
        attempt_reconnect_interval=15,
        disable_sessions=True
    )
    return await RobotClient.at_address(location, opts)

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

    while True:
        try:
            robot = await connect(location, secret)
            board = Board.from_robot(robot, "board")
            gpio = await board.gpio_pin_by_name("15")
            await gpio.set(True)
            time.sleep(3)
            await gpio.set(False)

            # The final method makes a call that expects a response in order to return properly,
            # however, if the power is successfully set to Deep Sleep, the device will not be able
            # to respond, throwing an exception.
            try: 
                await board.set_power_mode(PowerMode.POWER_MODE_OFFLINE_DEEP, duration=delta)
            except Exception as _:
                logging.info("success!")
                return
        except Exception as e:
            logging.error(e)

    # As the connection would time out if successfully put to deep sleep, `robot.close()`
    # is not necessary and would fail.

if __name__ == '__main__':
    asyncio.run(main())
    sys.exit()
 
