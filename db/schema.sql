\restrict 4DbGygQMpxSLNE0LuDOVTkDKCaPN7xV7TMgn3ciBQcgKPeU1d1j3C6AGk2H8Ttt

-- Dumped from database version 17.7
-- Dumped by pg_dump version 17.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: -
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: alert_condition_type; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.alert_condition_type AS ENUM (
    'greater_than',
    'less_than',
    'equals'
);


--
-- Name: alert_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.alert_status AS ENUM (
    'pending',
    'sent',
    'failed'
);


--
-- Name: device_status; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.device_status AS ENUM (
    'online',
    'offline',
    'maintenance'
);


--
-- Name: user_role; Type: TYPE; Schema: public; Owner: -
--

CREATE TYPE public.user_role AS ENUM (
    'admin',
    'user',
    'viewer'
);


--
-- Name: log_audit_event(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.log_audit_event() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    old_row JSONB := NULL;
    new_row JSONB := NULL;
    rec_id TEXT;
BEGIN
    IF (TG_OP = 'DELETE') THEN
        rec_id := OLD.id::TEXT;
        old_row := to_jsonb(OLD);
    ELSIF (TG_OP = 'UPDATE') THEN
        rec_id := NEW.id::TEXT;
        old_row := to_jsonb(OLD);
        new_row := to_jsonb(NEW);
    ELSIF (TG_OP = 'INSERT') THEN
        rec_id := NEW.id::TEXT;
        new_row := to_jsonb(NEW);
    END IF;

    INSERT INTO audit_log (table_name, record_id, operation, old_values, new_values)
    VALUES (TG_TABLE_NAME::TEXT, rec_id, TG_OP, old_row, new_row);

    RETURN NULL;
END;
$$;


--
-- Name: update_updated_at_column(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.update_updated_at_column() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: alert; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.alert (
    id integer NOT NULL,
    rule_id integer NOT NULL,
    triggered_at timestamp with time zone DEFAULT now() NOT NULL,
    sent_to character varying(255) NOT NULL,
    status public.alert_status NOT NULL
);


--
-- Name: alert_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.alert_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: alert_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.alert_id_seq OWNED BY public.alert.id;


--
-- Name: alert_rule; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.alert_rule (
    id integer NOT NULL,
    reservoir_id integer NOT NULL,
    condition_type public.alert_condition_type NOT NULL,
    threshold double precision NOT NULL,
    is_active boolean DEFAULT true,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: alert_rule_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.alert_rule_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: alert_rule_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.alert_rule_id_seq OWNED BY public.alert_rule.id;


--
-- Name: audit_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.audit_log (
    id integer NOT NULL,
    table_name text NOT NULL,
    record_id text NOT NULL,
    operation text NOT NULL,
    old_values jsonb,
    new_values jsonb,
    changed_at timestamp with time zone DEFAULT now()
);


--
-- Name: audit_log_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.audit_log_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: audit_log_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.audit_log_id_seq OWNED BY public.audit_log.id;


--
-- Name: device; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.device (
    id integer NOT NULL,
    user_id integer NOT NULL,
    reservoir_id integer,
    name character varying(255) NOT NULL,
    api_key character varying(255) NOT NULL,
    status public.device_status DEFAULT 'offline'::public.device_status NOT NULL,
    last_seen timestamp with time zone,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);


--
-- Name: device_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.device_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: device_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.device_id_seq OWNED BY public.device.id;


--
-- Name: measurement; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.measurement (
    "time" timestamp with time zone NOT NULL,
    device_id integer NOT NULL,
    value double precision NOT NULL
);


--
-- Name: reservoir; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.reservoir (
    id integer NOT NULL,
    user_id integer NOT NULL,
    name character varying(255) NOT NULL,
    description text,
    capacity double precision NOT NULL,
    location character varying(255),
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    CONSTRAINT chk_capacity_positive CHECK ((capacity > (0)::double precision))
);


--
-- Name: reservoir_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.reservoir_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: reservoir_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.reservoir_id_seq OWNED BY public.reservoir.id;


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying NOT NULL
);


--
-- Name: system_settings; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.system_settings (
    id integer DEFAULT 1 NOT NULL,
    maintenance_mode boolean DEFAULT false NOT NULL,
    registration_enabled boolean DEFAULT true NOT NULL,
    default_data_retention_days integer DEFAULT 30 NOT NULL,
    CONSTRAINT chk_retention_positive CHECK ((default_data_retention_days > 0)),
    CONSTRAINT chk_singleton CHECK ((id = 1))
);


--
-- Name: user; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public."user" (
    id integer NOT NULL,
    email character varying(255) NOT NULL,
    hashed_password character varying(255) NOT NULL,
    first_name character varying(255),
    last_name character varying(255),
    role public.user_role DEFAULT 'user'::public.user_role NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now(),
    is_banned boolean DEFAULT false NOT NULL,
    ban_reason text,
    CONSTRAINT chk_email_format CHECK (((email)::text ~~ '%@%'::text))
);


--
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.user_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.user_id_seq OWNED BY public."user".id;


--
-- Name: alert id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert ALTER COLUMN id SET DEFAULT nextval('public.alert_id_seq'::regclass);


--
-- Name: alert_rule id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert_rule ALTER COLUMN id SET DEFAULT nextval('public.alert_rule_id_seq'::regclass);


--
-- Name: audit_log id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_log ALTER COLUMN id SET DEFAULT nextval('public.audit_log_id_seq'::regclass);


--
-- Name: device id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device ALTER COLUMN id SET DEFAULT nextval('public.device_id_seq'::regclass);


--
-- Name: reservoir id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reservoir ALTER COLUMN id SET DEFAULT nextval('public.reservoir_id_seq'::regclass);


--
-- Name: user id; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user" ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- Name: alert alert_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert
    ADD CONSTRAINT alert_pkey PRIMARY KEY (id);


--
-- Name: alert_rule alert_rule_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert_rule
    ADD CONSTRAINT alert_rule_pkey PRIMARY KEY (id);


--
-- Name: audit_log audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.audit_log
    ADD CONSTRAINT audit_log_pkey PRIMARY KEY (id);


--
-- Name: device device_api_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device
    ADD CONSTRAINT device_api_key_key UNIQUE (api_key);


--
-- Name: device device_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device
    ADD CONSTRAINT device_pkey PRIMARY KEY (id);


--
-- Name: measurement measurement_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.measurement
    ADD CONSTRAINT measurement_pkey PRIMARY KEY ("time", device_id);


--
-- Name: reservoir reservoir_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reservoir
    ADD CONSTRAINT reservoir_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: system_settings system_settings_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.system_settings
    ADD CONSTRAINT system_settings_pkey PRIMARY KEY (id);


--
-- Name: user user_email_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_email_key UNIQUE (email);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: idx_measurement_device_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_measurement_device_time ON public.measurement USING btree (device_id, "time" DESC);


--
-- Name: alert_rule audit_alert_rule_changes; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER audit_alert_rule_changes AFTER INSERT OR DELETE OR UPDATE ON public.alert_rule FOR EACH ROW EXECUTE FUNCTION public.log_audit_event();


--
-- Name: device audit_device_changes; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER audit_device_changes AFTER INSERT OR DELETE OR UPDATE ON public.device FOR EACH ROW EXECUTE FUNCTION public.log_audit_event();


--
-- Name: system_settings audit_system_settings_changes; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER audit_system_settings_changes AFTER UPDATE ON public.system_settings FOR EACH ROW EXECUTE FUNCTION public.log_audit_event();


--
-- Name: user audit_user_changes; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER audit_user_changes AFTER INSERT OR DELETE OR UPDATE ON public."user" FOR EACH ROW EXECUTE FUNCTION public.log_audit_event();


--
-- Name: alert_rule update_alert_rule_modtime; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_alert_rule_modtime BEFORE UPDATE ON public.alert_rule FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: device update_device_modtime; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_device_modtime BEFORE UPDATE ON public.device FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: reservoir update_reservoir_modtime; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_reservoir_modtime BEFORE UPDATE ON public.reservoir FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: user update_user_modtime; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER update_user_modtime BEFORE UPDATE ON public."user" FOR EACH ROW EXECUTE FUNCTION public.update_updated_at_column();


--
-- Name: alert alert_rule_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert
    ADD CONSTRAINT alert_rule_id_fkey FOREIGN KEY (rule_id) REFERENCES public.alert_rule(id) ON DELETE CASCADE;


--
-- Name: alert_rule alert_rule_reservoir_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.alert_rule
    ADD CONSTRAINT alert_rule_reservoir_id_fkey FOREIGN KEY (reservoir_id) REFERENCES public.reservoir(id) ON DELETE CASCADE;


--
-- Name: device device_reservoir_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device
    ADD CONSTRAINT device_reservoir_id_fkey FOREIGN KEY (reservoir_id) REFERENCES public.reservoir(id) ON DELETE SET NULL;


--
-- Name: device device_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.device
    ADD CONSTRAINT device_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE;


--
-- Name: measurement measurement_device_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.measurement
    ADD CONSTRAINT measurement_device_id_fkey FOREIGN KEY (device_id) REFERENCES public.device(id) ON DELETE CASCADE;


--
-- Name: reservoir reservoir_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.reservoir
    ADD CONSTRAINT reservoir_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

\unrestrict 4DbGygQMpxSLNE0LuDOVTkDKCaPN7xV7TMgn3ciBQcgKPeU1d1j3C6AGk2H8Ttt


--
-- Dbmate schema migrations
--

INSERT INTO public.schema_migrations (version) VALUES
    ('20251203144947'),
    ('20251203144950'),
    ('20251203144954'),
    ('20251203144957'),
    ('20251203145000'),
    ('20251203145003'),
    ('20251203145007'),
    ('20251203145010'),
    ('20251205110540');
