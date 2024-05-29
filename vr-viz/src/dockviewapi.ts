import {AddPanelOptions, DockviewApi} from "dockview";

const gyroSettings = {
    "id": "gyro",
    "component": "gyro",
    "title": "Gyroscope",
}

const vrdcSettings = {
    "id": "vrdc",
    "component": "vrdc",
    "title": "VR Distance Configuration",
}

function openOrCreatePanel(api: DockviewApi, panelDefinition: AddPanelOptions) {
    const panelId = panelDefinition.id;
    const panel = api.getPanel(panelId);

    if (panel === undefined) {
        api.addPanel(panelDefinition);
    } else {
        panel.api.setActive()
    }
}

export function openGyroTab(api: DockviewApi) {
    openOrCreatePanel(api, gyroSettings);
}

export function openVrDistanceConfigurationTab(api: DockviewApi) {
    openOrCreatePanel(api, vrdcSettings);
}