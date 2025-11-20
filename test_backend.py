#!/usr/bin/env python3
"""
Quick test backend for Anti-Detect MVP
This is just for testing - production uses Rust backend
"""

from flask import Flask, request, jsonify
from flask_cors import CORS
import psycopg2
import bcrypt
import jwt
import datetime
from uuid import uuid4

app = Flask(__name__)
CORS(app)

# Database connection
DB_CONFIG = {
    'host': 'localhost',
    'port': 5432,
    'database': 'antidetect_db',
    'user': 'antidetect_user',
    'password': 'antidetect123'
}

JWT_SECRET = 'your-secret-key'

def get_db():
    return psycopg2.connect(**DB_CONFIG)

@app.route('/')
def index():
    return jsonify({'message': 'Anti-Detect Browser Backend API (Test Server)'})

@app.route('/api/auth/login', methods=['POST'])
def login():
    data = request.json
    email = data.get('email')
    password = data.get('password')
    
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT id, email, password_hash, role, is_active FROM users WHERE email = %s", (email,))
    user = cur.fetchone()
    cur.close()
    conn.close()
    
    if not user:
        return jsonify({'error': 'Invalid credentials'}), 401
    
    user_id, user_email, password_hash, role, is_active = user
    
    if not bcrypt.checkpw(password.encode(), password_hash.encode()):
        return jsonify({'error': 'Invalid credentials'}), 401
    
    if not is_active:
        return jsonify({'error': 'Account is disabled'}), 403
    
    token = jwt.encode({
        'sub': str(user_id),
        'email': user_email,
        'role': role,
        'exp': datetime.datetime.utcnow() + datetime.timedelta(hours=24)
    }, JWT_SECRET, algorithm='HS256')
    
    return jsonify({
        'token': token,
        'user': {
            'id': str(user_id),
            'email': user_email,
            'role': role
        }
    })

@app.route('/api/users', methods=['GET'])
def get_users():
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT id, email, role, is_active, created_at, updated_at FROM users ORDER BY created_at DESC")
    users = cur.fetchall()
    cur.close()
    conn.close()
    
    return jsonify([{
        'id': str(u[0]),
        'email': u[1],
        'role': u[2],
        'is_active': u[3],
        'created_at': u[4].isoformat() if u[4] else None,
        'updated_at': u[5].isoformat() if u[5] else None
    } for u in users])

@app.route('/api/licenses', methods=['GET'])
def get_licenses():
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT id, license_key, plan, max_profiles, user_id, is_active, expires_at, activated_at, created_at, updated_at FROM licenses ORDER BY created_at DESC")
    licenses = cur.fetchall()
    cur.close()
    conn.close()
    
    return jsonify([{
        'id': str(l[0]),
        'license_key': l[1],
        'plan': l[2],
        'max_profiles': l[3],
        'user_id': str(l[4]) if l[4] else None,
        'is_active': l[5],
        'expires_at': l[6].isoformat() if l[6] else None,
        'activated_at': l[7].isoformat() if l[7] else None,
        'created_at': l[8].isoformat() if l[8] else None,
        'updated_at': l[9].isoformat() if l[9] else None
    } for l in licenses])

@app.route('/api/licenses', methods=['POST'])
def create_license():
    data = request.json
    plan = data.get('plan', 'basic')
    max_profiles = data.get('max_profiles', 3)
    
    # Generate license key
    license_key = f"{uuid4().hex[:8].upper()}-{uuid4().hex[:8].upper()}-{uuid4().hex[:8].upper()}-{uuid4().hex[:8].upper()}"
    
    # Calculate expiry
    days = {'trial': 7, 'basic': 30, 'pro': 90, 'enterprise': 365}
    expires_at = datetime.datetime.utcnow() + datetime.timedelta(days=days.get(plan, 30))
    
    conn = get_db()
    cur = conn.cursor()
    cur.execute("""
        INSERT INTO licenses (license_key, plan, max_profiles, expires_at, is_active)
        VALUES (%s, %s, %s, %s, true)
        RETURNING id, license_key, plan, max_profiles, user_id, is_active, expires_at, activated_at, created_at, updated_at
    """, (license_key, plan, max_profiles, expires_at))
    
    new_license = cur.fetchone()
    conn.commit()
    cur.close()
    conn.close()
    
    return jsonify({
        'id': str(new_license[0]),
        'license_key': new_license[1],
        'plan': new_license[2],
        'max_profiles': new_license[3],
        'user_id': str(new_license[4]) if new_license[4] else None,
        'is_active': new_license[5],
        'expires_at': new_license[6].isoformat() if new_license[6] else None,
        'activated_at': new_license[7].isoformat() if new_license[7] else None,
        'created_at': new_license[8].isoformat() if new_license[8] else None,
        'updated_at': new_license[9].isoformat() if new_license[9] else None
    }), 201

@app.route('/api/profiles', methods=['GET'])
def get_profiles():
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT id, user_id, name, user_agent, screen_resolution, timezone, language, created_at, updated_at FROM profiles ORDER BY created_at DESC")
    profiles = cur.fetchall()
    cur.close()
    conn.close()
    
    return jsonify([{
        'id': str(p[0]),
        'user_id': str(p[1]),
        'name': p[2],
        'user_agent': p[3],
        'screen_resolution': p[4],
        'timezone': p[5],
        'language': p[6],
        'created_at': p[7].isoformat() if p[7] else None,
        'updated_at': p[8].isoformat() if p[8] else None
    } for p in profiles])

@app.route('/api/proxies', methods=['GET'])
def get_proxies():
    conn = get_db()
    cur = conn.cursor()
    cur.execute("SELECT id, user_id, proxy_type, host, port, country, created_at, updated_at FROM proxies ORDER BY created_at DESC")
    proxies = cur.fetchall()
    cur.close()
    conn.close()
    
    return jsonify([{
        'id': str(p[0]),
        'user_id': str(p[1]),
        'proxy_type': p[2],
        'host': p[3],
        'port': p[4],
        'country': p[5],
        'created_at': p[6].isoformat() if p[6] else None,
        'updated_at': p[7].isoformat() if p[7] else None
    } for p in proxies])

if __name__ == '__main__':
    print("🚀 Starting Test Backend Server on http://0.0.0.0:3000")
    print("📧 Login: admin@antidetect.local / admin123")
    app.run(host='0.0.0.0', port=3000, debug=True)
