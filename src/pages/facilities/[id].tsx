import { GetStaticPaths, GetStaticProps } from "next";
import { ParsedUrlQuery } from "querystring";
import { Facility } from "../../../lib/facility";

export interface Props {
    facility: Facility;
}

export interface Params extends ParsedUrlQuery {
    id: string;
}

export const getStaticPaths: GetStaticPaths<Params> = async () => {
    const { getFacilityIds } = await import('../../../lib/facility');
    const ids = await getFacilityIds();
    const paths = ids.map((id) => ({ params: { id } }));
    return {
        paths,
        fallback: false,
    };
};

export const getStaticProps: GetStaticProps<Props, Params> = async ({ params }) => {
    const { getFacility } = await import('../../../lib/facility');
    const facility = await getFacility(params!.id);
    return {
        props: {
            facility,
        },
    };
};

export const FacilityView = ({ facility }: Props) => {
    return (
        <p>
            <p>{facility.name}</p>
            <p>{facility.kind}</p>
            <p>lat: {facility.coordinate.lat}</p>
            <p>lon: {facility.coordinate.lon}</p>
            {facility.goshuinList.map((goshuin, i) => (
                <img key={i} width='30%' src={goshuin.pictureUrls[0]} />
            ))}
            {facility.attachments && facility.attachments.map((attachment, i) => (
                <img key={i} width='30%' src={attachment.mediaUrl} />
            ))}
        </p>
    );
};

export default FacilityView;
