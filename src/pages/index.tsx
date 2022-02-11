import { GetStaticProps } from "next";
import Link from 'next/link';
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

const Index = ({ facilities }: Props) => {
    return (
        <div>
            <ul>
                {facilities.map(({ id, name }) => (
                    <li key={id}>
                        <Link
                            href={`/facilities/${id}`}
                        >
                            {name}
                        </Link>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default Index;
