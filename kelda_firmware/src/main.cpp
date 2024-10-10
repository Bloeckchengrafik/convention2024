#include <Arduino.h>
#include "ftSwarmRS.h"
#include "SwOSCLI.h"
#include "cnDevices.h"

#define KELDA

SwOSCLI *cli;

void setup() {
    Serial.begin(115200);

    #ifndef KELDA
    firmware();
    ESP.restart();
    return;
    #endif

    FtSwarmSerialNumber_t num = ftSwarm.begin();
    cli = new SwOSCLI();

    beginThrottle();

    printf("\n\nRunning ftConvention2024 Special Edition\n\n");

}

void GlobalCommandExecutor::run(CLICmd_t cmd, SwOSCLIParameter *param, int maxParam) {
    switch (cmd)
    {
    case CLICMD_getThrottle:
        printf("R: %d\n", throttleValue);
        break;
    default:
        printf("R: ok\n");
        break;
    }
}

void loop () {
    #ifndef KELDA
    delay(250);
    return;
    #endif

    cli->readln();
    if (!cli->eval()) {
        ESP.restart();
    }
}
