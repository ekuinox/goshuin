import { GetStaticProps } from "next";
import Link from 'next/link';

export interface Props {
    ids: Array<string>;
}

export const getStaticProps: GetStaticProps<Props> = async () => {
    const { getFacilityIds } = await import('../../lib/facility');
    const ids = await getFacilityIds();

    return {
        props: {
            ids,
        },
    };
};

const Index = ({ ids }: Props) => {
    return (
        <div>
            {ids.map((id) => (
                <Link
                    key={id}
                    href={`/facilities/${id}`}
                >
                    {id}
                </Link>
            ))}
        </div>
    );
};

export default Index;
