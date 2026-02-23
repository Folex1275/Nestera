import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import request from 'supertest';
import { AppModule } from './../src/app.module';
import { PrismaService } from './../src/prisma/prisma.service';

// Provide required env vars for validation schema BEFORE module loading
process.env.NODE_ENV = 'test';
process.env.PORT = '3001';
process.env.DATABASE_URL = 'postgresql://user:pass@localhost:5432/test';
process.env.JWT_SECRET = 'super-secret-key-for-testing-purposes_long_enough';
process.env.JWT_EXPIRATION = '1h';
process.env.SOROBAN_RPC_URL = 'https://soroban-testnet.stellar.org';
process.env.HORIZON_URL = 'https://horizon-testnet.stellar.org';
process.env.CONTRACT_ID = 'CC...';
process.env.STELLAR_WEBHOOK_SECRET = 'webhook-secret-long-enough-123456';

describe('Authentication (e2e)', () => {
    let app: INestApplication;

    const mockUser = {
        id: 'user-123',
        email: `test@example.com`,
        password: 'hashed-password', // This is the stored hashed password
        name: 'Test User',
    };

    // This user object is used for sending requests, with a plain password
    const requestUser = {
        email: mockUser.email,
        password: 'password123', // This is the plain password sent in requests
        name: mockUser.name,
    };

    const mockPrismaService = {
        user: {
            findUnique: jest.fn().mockImplementation((args) => {
                if (args.where.email === mockUser.email) return Promise.resolve(mockUser);
                return Promise.resolve(null);
            }),
            create: jest.fn().mockImplementation((args) =>
                Promise.resolve({ id: 'new-id', ...args.data })
            ),
        },
        $connect: jest.fn().mockResolvedValue(undefined),
    };

    let token: string;

    beforeAll(async () => {
        const moduleFixture: TestingModule = await Test.createTestingModule({
            imports: [AppModule],
        })
            .overrideProvider(PrismaService)
            .useValue(mockPrismaService)
            .compile();

        app = moduleFixture.createNestApplication();
        app.useGlobalPipes(new ValidationPipe());
        await app.init();
    });

    afterAll(async () => {
        await app.close();
    });

    it('/auth/register (POST)', () => {
        return request(app.getHttpServer())
            .post('/auth/register')
            .send({
                email: 'new-user@example.com',
                password: 'password123',
                name: 'New User'
            })
            .expect(201)
            .then((res) => {
                expect(res.body).toHaveProperty('accessToken');
                expect(res.body.user.email).toBe('new-user@example.com');
            });
    });

    it('/auth/login (POST)', async () => {
        // Generate a real hash for the mock user so bcrypt.compare works
        const hashedPassword = await import('bcrypt').then(b => b.hash('password123', 10));
        mockPrismaService.user.findUnique.mockResolvedValueOnce({ ...mockUser, password: hashedPassword });

        return request(app.getHttpServer())
            .post('/auth/login')
            .send({
                email: mockUser.email,
                password: 'password123',
            })
            .expect(200)
            .then((res) => {
                expect(res.body).toHaveProperty('accessToken');
                token = res.body.accessToken;
            });
    });

    it('/users/me (GET) - Protected Route with @CurrentUser', () => {
        // mock findById in UserService via prisma
        mockPrismaService.user.findUnique.mockResolvedValueOnce(mockUser);

        return request(app.getHttpServer())
            .get('/users/me')
            .set('Authorization', `Bearer ${token}`)
            .expect(200)
            .then((res) => {
                expect(res.body.email).toBe(mockUser.email);
            });
    });

    it('/auth/login (POST) - Invalid Credentials', () => {
        return request(app.getHttpServer())
            .post('/auth/login')
            .send({
                email: mockUser.email,
                password: 'wrongpassword',
            })
            .expect(401);
    });
});
