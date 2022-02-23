import Head from 'next/head';
import dynamic from 'next/dynamic';
import { GetStaticProps } from "next";
import { Facility } from "../../lib/facility";

export interface Props {
    facilities: Array<Facility>;
}

export const getStaticProps: GetStaticProps<Props> = async () => {
    const { getFacilityIds, getFacility } = await import('../../lib/facility');
    const ids = await getFacilityIds();
    const facilities = await Promise.all(ids.map(getFacility));
    return {
        props: {
            facilities,
        },
    };
};

const Map = dynamic(
    async () => import('../components/map').then(({ Map }) => Map),
    { ssr: false }
);

const Index = ({ facilities }: Props) => {
    return (
        <div>
            <Head>
                <title>
                    御朱印マップ
                </title>
            </Head>
            <Map facilities={facilities} />
        </div>
    );
};

export default Index;
