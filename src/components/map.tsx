import Link from 'next/link';
import { MapContainer, Marker, Popup, TileLayer, useMap } from 'react-leaflet';
import { Facility } from "../../lib/facility";
import { useEffect } from 'react';
import { LatLngExpression, Icon } from 'leaflet';
import "leaflet/dist/leaflet.css";

import markerIcon2x from "leaflet/dist/images/marker-icon-2x.png";
import markerIcon from "leaflet/dist/images/marker-icon.png";
import markerShadow from "leaflet/dist/images/marker-shadow.png";

// @ts-ignore
delete Icon.Default.prototype._getIconUrl;

Icon.Default.mergeOptions({
    iconUrl: markerIcon.src,
    iconRetinaUrl: markerIcon2x.src,
    shadowUrl: markerShadow.src,
});

const DEFAULT_CENTER: LatLngExpression = [34.677578, 135.415826]; // 大阪市
const DEFAULT_ZOOM = 13;

export interface MapProps {
    facilities: Array<Facility>;
}

const SetCurrentLocationToCenter = () => {
    const map = useMap();
    useEffect(() => {
        if (window?.navigator?.geolocation == null) {
            return;
        }
        navigator.geolocation.getCurrentPosition(({ coords }) => {
            map.setView([coords.latitude, coords.longitude]);
        });
    }, [map]);

    return (
        <></>
    );
};

const FacilityMarker = ({ facility: { id, name, coordinate, goshuinList } }: { facility: Facility }) => {
    return (
        <Marker key={id} position={[coordinate.lat, coordinate.lon]}>
            <Popup>
                <Link href={`/facilities/${id}`}>
                    {name}
                </Link>
                {goshuinList.length > 0 && (
                    <img
                        width='100px'
                        src={goshuinList[0].pictureUrls[0]}
                    />
                )}
            </Popup>
        </Marker>
    );
};

export const Map: React.FC<MapProps> = ({ facilities }) => {
    return (
        <MapContainer
            center={DEFAULT_CENTER}
            zoom={DEFAULT_ZOOM}
            style={{ height: "80vh", width: "100%" }}
        >
            <TileLayer
                attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
                url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
            />
            {facilities.map((item) => <FacilityMarker facility={item} />)}
            <SetCurrentLocationToCenter />
        </MapContainer>
    );
};
