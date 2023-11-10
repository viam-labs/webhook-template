import asyncio
import sys
from datetime import timedelta

from viam.robot.client import RobotClient
from viam.components.board import Board
from viam.rpc.dial import Credentials, DialOptions
from viam.proto.component.board import PowerMode
from viam.logging import getLogger

LOGGER = getLogger("example-script")

async def connect(robot_url,api_key_id, api_key):
    opts = RobotClient.Options(
        refresh_interval=0,
        check_connection_interval=0,
        attempt_reconnect_interval=0,
        disable_sessions=True,
        dial_options=DialOptions.with_api_key(api_key_id=api_key_id,api_key=api_key)
    )
    return await RobotClient.at_address(robot_url, opts)

async def main():
    robot_url = sys.argv[1]
    api_key_id = sys.argv[2]
    api_key = sys.argv[3]

    # duration of deep sleep until device's next call to webhook
    delta = timedelta(
        minutes=1,
    )

    # sometimes the connection can fail, we want to try a couple of time before giving up
    n_try = 10
    while n_try:
        try:
            robot = await connect(robot_url,api_key_id,api_key)
            break;
        except Exception as e:
            n_try = n_try - 1;
            LOGGER.info("couldn't connect to robot reason {} trying again for {} times".format(e, n_try));
            if n_try == 0:
                LOGGER.error("couldn't connect to robot after 10 tries giving up")
                return

    LOGGER.info(f"Resources {robot.resource_names}")

    try:
        await board.set_power_mode(PowerMode.POWER_MODE_OFFLINE_DEEP, duration=delta, timeout = 5)
    except:
        LOGGER.info("set power mode doesn't return assuming everything went well")

    # The ESP32 is sleeping at this point so we aren't able to gracefully close the connection
    # we therefore quit immediately
    sys.exit(0)


if __name__ == '__main__':
    try:
        asyncio.run(main())
    except Exception as e:
        LOGGER.info(f"closing script after exception {e}")
    sys.exit(0)
 
