package main

import (
  "fmt"
  "context"
  "time"
  "os"

  "github.com/edaniels/golog"
  "go.viam.com/rdk/robot/client"
  "go.viam.com/rdk/utils"
  "go.viam.com/utils/rpc"
  "go.viam.com/rdk/components/board"
  boardpb "go.viam.com/api/component/board/v1"

)

func main() {
  // argv[1] = fqdn
  fqdn := os.Args[1]
  // argv[2] = secret
  secret := os.Args[2]
  fmt.Println("Gohook - FQDN: ", fqdn, " - Secret: ", secret)

  logger := golog.NewDebugLogger("client")
  robot, err := client.New(
      context.Background(),
      fqdn,
      logger,
      client.WithDialOptions(rpc.WithCredentials(rpc.Credentials{
          Type:    utils.CredentialsTypeRobotLocationSecret,
          Payload: secret,
      })),
  )

  if err != nil {
      logger.Fatal(err)
  }
  
  // Note that the pin supplied is a placeholder. Please change this to a valid pin. 
  myboard, err:= board.FromRobot(robot, "board")
  if err!=nil {
    logger.Error(err)
  }

  // analog reader
  reader, isConnected:= myboard.AnalogReaderByName("soil")
  if !isConnected {
    logger.Error("Failed to get soil reader")
  }

  // pump pin
  pin, err:= myboard.GPIOPinByName("15")
  if err!=nil {
    logger.Error(err)
  }

  logger.Debugf("GPIOPinByName return value: %v", pin)

  // get dryness
  dryness, err := reader.Read(context.Background(), nil)
  if err!=nil {
    logger.Error(err)
  }

  fmt.Println("dryness: ", dryness)

  if dryness > 100 {
    pin.Set(context.Background(), true, nil)
    time.Sleep(3 * time.Second)
    pin.Set(context.Background(), false, nil)
  }

  dur := time.Second * 60

  fmt.Println("sleeping for: ", dur)
  myboard.SetPowerMode(context.Background(), boardpb.PowerMode_POWER_MODE_OFFLINE_DEEP, &dur)

  // trying to close an abandoned (by esp32) connection
  err = robot.Close(context.Background())
  if err != nil {
    logger.Error("failed to close. may be sleeping already.")
  }
}

