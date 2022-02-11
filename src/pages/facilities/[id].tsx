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
    const paths = ids.map((id) => ({ params: { id }}));
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
            <img src={facility.goshuinList[0].pictureUrls[0]} />
        </p>
    );
};

export default FacilityView;
