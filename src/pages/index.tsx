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
            <ul>
                {ids.map((id) => (
                    <li key={id}>
                        <Link
                            href={`/facilities/${id}`}
                        >
                            {id}
                        </Link>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default Index;
